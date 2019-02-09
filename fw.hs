-- will be a floyd_warshall

infinities :: Int -> [[Int]]
infinities n = take n (repeat (take n (repeat (maxBound :: Int))))  

