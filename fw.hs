-- will be a floyd warshall

import qualified Data.List.Split as Split

chunksOf n lst = Split.chunksOf n lst


infinities :: Int -> [[Int]]
infinities n = take n (repeat (take n (repeat (maxBound :: Int))))  


initial 0 lst = []
initial n lst = (head lst):(initial (n-1) (tail lst))


chgLine :: Int -> Int -> [Int] -> [Int]
chgLine pos val line = (initial pos line) ++ val : (drop (pos+1) line)


zeroDiagInternal :: Int -> [[Int]] -> [[Int]]
zeroDiagInternal 0 matrix = matrix
zeroDiagInternal pos matrix = chgLine pos 0 (last matrix):(zeroDiagInternal (pos-1) (init matrix))

zeroDiag :: [[Int]] -> [[Int]]
zeroDiag matrix = reverse (zeroDiagInternal (length matrix) matrix)


chgMatrixInternal :: Int -> Int -> Int -> [[Int]] -> [[Int]]
chgMatrixInternal 0 j val (x:xs) = (chgLine j val x):xs
chgMatrixInternal i j val (x:xs) = x:(chgMatrixInternal (i-1) j val xs)

chgMatrix :: Int -> Int -> Int -> [[Int]] -> [[Int]]
chgMatrix i j _ lst
    | i >= len || i < 0 = error "Line index out of range."
    | j >= len || j < 0 = error "Column index out of range."
    where len = length lst
chgMatrix i j val matrix = chgMatrixInternal i j val matrix


-- getMin matrix i j k = min (matrix !! i !! j) ((matrix !! i !! k)+(matrix !! k !! j))
-- setMinInternal matrix i j k = chgMatrix i j (getMin matrix i j k)