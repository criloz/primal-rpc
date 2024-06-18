(Version "0.1")

(def IdempotentHandling
  (@ :ulid "01J00F180ZAA09G763X939K3MH")
  (enum
   ;; accept the task even if there is one unassigned task with the
   ;; idempotent key and update the accepted_at key but ignore the payload
   (@1)
   :Accept
   ;; reject the task if the idempotent key is already present within the unassigned task set with a 409 conflict
   (@2)
   :Reject
   ;; accept the task but do not update the accepted_at field or the payload if there is an unassigned task with the same idempotent key
   (@3)
   :Ignore
   ;; replace the task payload if the idempotent key is already present within the unassigned task set, this will update the accepted_at field too
   (@4)
   :Replace
   (@5)
   ;; only allow one task with the same idempotent key and queue name
   :Unique))


(def TaskOrderKey
  (@ :ulid "01J0EHYENMFDTED3PWC2RT96B9")
  (enum
   (@1)
   ;; specify if the task will be ordered in the queue using the create_at key
   (@2)
   :CreatedAt,
   ;; specify if the task will be ordered in the queue using the accepted_at key
   (@3)
   :AcceptAt))

(def TaskOrder
  (id "01J00EXDVE22JDPE393RJDHC0J")
  (enum
   [;;; Last in first out
     :Lifo
     {:key (default TaskOrderKey :CreatedAt)}
     ;;; First in first out
     :Fifo
     {:key (default TaskOrderKey :CreatedAt)}]))


(def IdempotentConfig
  (id "01J00EZQ5B2R77YY22Z34RVNS2")
  {;;; When a queue is configured with `is_key_required` all the task should provide an idempotent key
    :is_key_required (default bool false)
    ;;; The handling of idempotent tasks
    :handling        (default IdempotentHandling :Accept)
    ;;; The time in milliseconds to wait before the task can be processed after being accepted
    ;;; Combined with idempotence key in the [Accept, Replace] cases can be used to debounce the task execution
    :delay           (default duration_ms 0)})

(def QueueSettings
  {;;; The name of the queue
    :order       (default TaskOrder :Fifo)
    ;;; The order in which tasks are processed
    :partitioned (default bool false),
    ;;; Whether the queue is partitioned
    :idempotency (default IdempotentConfig _)})

(def QueueName (subset string #"^[a-z][a-z0-9-]*[a-z0-9]"))

(def Queue
  {;;; The nameIof the queue
    :id         (generated :server sqid),
    :created_at (generated :server datetime_utc)
    :name       (generated :client QueueName)
    ;;; The settings of the queue
    :settings   (default QueueSettings *)})

(def Queues
  (collection (for Queue) (identifier :name) (identifier :id)))


(def info (operation :arguments (exist Queues) :success Queue))
(def upsert
  (operation
   :arguments (create Queues)
   :success
              (enum
               [;;; The queue was created
                 :Created
                 Queue
                 :Acccepted
                 Queue])
   :errors
              (enum
               [;;; this error hapens if the task already exists and the settings are different
                 :Conflict
                 {:http_status status.conflict,
                  :message     "The queue already exists with different settings",
                  :suggestions ["Use the `update` operation to update the queue settings",
                                "delete the queue and create a new one with the desired settings"]}])))

;;; The task status
(def TaskStatus
  (enum
   [;;; The task is unassigned
     :Unassigned
     ;;; The task is assigned to a worker
     :Assigned
     ;;; The task is in progress
     :InProgress
     ;;; The task is completed
     :Completed
     ;;; The task is being retried
     :Retrying
     ;;; The task has failed
     :Failed]))

(def Task
  {;;; The id of the task
    :id              #idempotent_track #server sqid,
    ;;; The idempotency key of the task
    :idempotency_key (optional string)
    ;;; The partition key of the task
    :partition_key   (optional string),
    ;;; The payload of the task
    :payload         json
    ;;; The status of the task
    :status          (default TaskStatus :Unassigned)
    ;;; The time the task was created
    :created_at      #idempotent_track (generated :server datetime_utc)
    ;;; The time the task was accepted by the scheduler
    :accepted_at     #idempotent_track #server datetime_utc
    ;;; The time the task was last updated
    :updated_at      :server
    datetime_utc})


(def WorkerStatus
  (enum
   [;;; The worker is available to accept tasks
     :Available
     ;;; The worker is busy processing a task
     :Busy
     ;;; The worker is no longer available
     :Deceased]))

;;; The worker entity
(def Worker
  (ulid "01J0C8Y5NP7A1QW71QMJFXG2S7")
  {;;; The id of the worker
    :id             #id #server sqid,
    ;;; The name of the worker
    :worker_name    #client #optional string,
    ;;; The status of the worker
    :worker_status  (default WorkerStatus :Available),
    ;;; The time the worker was created
    :created_at     #server datetime_utc,
    ;;; The time the worker was last updated
    :updated_at     #server datetime_utc,
    ;;; The time the worker last sent a heartbeat
    :last_heartbeat #server datetime_utc})

(def Workers (collection Worker (identifier :id)))

(def TaskRunStatus
  (enum
   [;;; The task is being processed
     :Working
     ;;; The task was successfully completed
     :Success
     ;;; The task failed
     :Failure]))
(def TaskRun
  {;;; The id of the task run
    :run_id       #server sqid,
    ;;; The status of the task run
    :run_status   #server (default TaskRunStatus :Working),
    ;;; The error message if the task failed
    :error        #optional #client json
    ;;; The time the task run started
    :started_at   #server datetime_utc
    ;;; The time the task run completed
    :completed_at (#optional #server datetime_utc)})

;;; The operation to add a task to the queue
(def enqueue (operation :arguments [(exist Queues), (create Tasks)]))
(def subscirbe
  (operation :arguments [(exist Queues), (exist Worker) | (create Worker)]))


(def dequeue
  (operation
   :arguments [(exist Queues), (exist Workers)])
  :success
  (enum
   [;;; The task was assigned to the worker
     :Assigned
     (+ {:http_status status.ok} TaskRun)
     ;;; There are no tasks to assign
     :NoTasks
     {:http_status status.no_content,
      :message     "There are no tasks to assign",
      :suggestions ["Check back later"]}
     ;;; The worker is not available
     :WorkerNotSubscribedToQueue
     {:http_status status.bad_request,
      :message     "The worker is not subscribed to the queue",
      :suggestions ["subscribe the worker to the queue first"]}]))

(def TaskRunManagmentError
  (enum
   :tag "code" ;
   [;;; The task run is no longer available
     :TaskRunCompleted
     {:failed bool}
     (error :status  :StatusCode:Conflict
            :message "The task run has failed",
            :suggest ["do not complete the task run again"]),
     :TaskRunFailed
     {:http_status status.bad_request,}
     :WorkerDead
     {:http_status status.bad_request,
      :message     "The worker is no longer available",
      :suggestions ["subscribe the worker to the queue first"]}
     ;;; The task run is not assigned to the worker
     :TaskRunNotAssignedToWorker
     {:http_status status.bad_request,
      :message     "The task run is not assigned to the worker",
      :suggestions ["assign the task run to the worker first"]}]))

(defn exist [])
(defn create [])
(defn route [])
(defn operation [])


;;; complete a task run if error is present the task run is marked as failed
(def enqueue
  (operation :arguments [(exist Queues), (create Task)]
             :success   (enum :tag "code" :Created Task :Acccepted Task)))

(def queues
  (route
   enqueue,))
(def complete
  (operation :arguments
                     [(update TaskRun [:error #optional _]),
                      (exist Workers),
                      (query {:dequeue (default bool false)})]
             :errors TaskRunManagmentError
             :success
                     (enum
                      [;;; The task run was successfully completed
                        :Completed
                        {:http_status status.accepted}
                        ;;; if dequeue is true the worker will be assigned a new task if available
                        :NextTask
                        (+ {:http_status status.created} TaskRun)])))




