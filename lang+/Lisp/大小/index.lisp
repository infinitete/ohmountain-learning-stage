;;;
;;; 返回两个数较大的一个
;;;
(defun get_max (a b)
  (if (> a b) a b))

(format t "~A ~%" (get_max 1 2))
(format t "~A ~%" (get_max 3 2))
(format t "~A ~%" (get_max 9 2))
