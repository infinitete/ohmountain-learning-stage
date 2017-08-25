;;;
;;; 累加器，如输入 100
;;; 则返回 1 + 2 + 3 + 4 + ... + 100 (5050)
;;;
(defun leijia (number)
  (if (null (numberp number)) 0
      (if (> number 1)
          (+ number (leijia (- number 1)))
          1)))

(format t "~A ~%" (leijia 10))
