(defun range (min max)
  (loop for i from min below max collect nil))

(defun create_list (len)
  (append (range 0 (- len 1)) '(t)))

(defun display (l)
  (loop for item in l do
    (if item
      (format t "#")
      (format t " ")))
    (format t "~%"))

(defun is-alive (a b c)
  (cond ((and a b c) nil)
        ((and a b (not c)) t)
        ((and a (not b) c) t)
        ((and a (not b) (not c)) nil)
        ((and (not a) (not b) c) t)
        ((and (not a) b (not c)) t)
        ((and (not a) (not b) c) t)
        ((and (not a) (not b) (not c)) nil)))

(defun idx-of (grid i)
  (mod (+ i (length grid)) (length grid)))

(defun update (grid)
  (loop :for i :from 0 :to (- (length grid) 1)
    :collect (let ((a (nth (idx-of grid (- i 1)) grid))
          (b (nth i grid))
          (c (nth (idx-of grid (+ i 1)) grid)))
      (is-alive a b c)) into new
    finally (return new)))

(let ((grid (create_list 120)))
  (loop
      (display grid)
      (setq grid (update grid))
      (sleep 0.09)))
