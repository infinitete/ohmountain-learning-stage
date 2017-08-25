;;;
;;; 列表过滤空值后求和
;;;
(defun summit (list)
  (if (listp list)
      (let ((list (remove nil list)))
        (apply #'+ list)
        )
      0)
  )

(summit '(1 2 3 nil 10 100 nil))

;;;
;;; 版本2
;;;
(defun summit1 (list)
  (if (listp list)
      (apply #'+ (remove nil list))
      0
      ))

(summit1 '(1 2 3 nil 101000 nil))
