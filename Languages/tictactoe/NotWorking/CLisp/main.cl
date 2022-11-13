(defun is_num (str)
  (if (typep str 'string)
    (numberp (read-from-string str)))
    ())

(defun get-input () (read-line))

(defun input (msg)
    (format t msg)
    (get-input))

(defparameter *spots* (list "0" "1" "2" "3" "4" "5" "6" "7" "8" "9" ))

(defun render (spots)
  (format t "~$ | ~$ | ~$~%~$ | ~$ | ~$~%~$ | ~$ | ~$~%"
  (nth 0 spots)
  (nth 1 spots)
  (nth 2 spots)
  (nth 3 spots)
  (nth 4 spots)
  (nth 5 spots)
  (nth 6 spots)
  (nth 7 spots)
  (nth 8 spots)
  (nth 9 spots)))

(defun setspot (idx sign spots)
  (setf (nth idx spots) sign)
  )


(print (input "number> "))
