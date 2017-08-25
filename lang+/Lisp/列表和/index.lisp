(defun summit (list)
  (if (listp list)
      (let ((list (remove nil list)))
        (apply #'+ list)
        )
      0)
  )

(summit '(1 2 3 nil 10 100 nil))
