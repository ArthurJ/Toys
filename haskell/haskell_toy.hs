
factorial :: Integer -> Integer
factorial n = product [1..n]


pythagorasTriples :: Int -> [(Int, Int, Int)]
pythagorasTriples sideSum = [(a,b,c) | c <- [1..quot sideSum 2],
                                       b <- [1..c],
                                       a <- [1..b],
                                a+b+c == sideSum, a^2+b^2==c^2]

rightTriangle :: Int -> [(Int, [(Int, Int, Int)])]
rightTriangle sideSum = [v | v <- [(x, pythagorasTriples x) | x <- [1..sideSum]], not (null (snd v))]


circunference :: Floating a => a -> a
circunference r = 2*pi*r


imcValor :: (Floating a) => a -> a -> a
imcValor altura peso = peso/altura^2

imcClass :: Float -> String
imcClass valor  | valor < 16 =  "Magreza grave"
                | valor < 17 =  "Magreza moderada"
                | valor < 18.5 ="Magreza leve"
                | valor < 25 =  "Saudável"
                | valor < 30 =  "Sobrepeso"
                | valor < 35 =  "Obesidade grau I"
                | valor < 40 =  "Obesidade grau II (Severa)"
                | otherwise =   "Obesidade grau III (Mórbida)"


calculaImc :: Float -> Float -> String
calculaImc altura peso = imcClass (imcValor altura peso)


max' :: (Ord a) => a -> a -> a
max' a b | a <= b = b
         | otherwise = a


initials :: String -> String -> String
initials (f:_) (l:_) = [f] ++ ". " ++ [l] ++ "."

initials' :: String -> String -> String
initials' fstname lstname = [f] ++ [l]
    where (f:_) = fstname
          (l:_) = lstname


maximum' :: (Ord a) => [a] -> a
maximum' [] = error "maximum of an empty list?"
maximum' [x] = x
maximum' (x:xs) = max x (maximum' xs)


replicate' :: Int -> x -> [x]
replicate' n x  | n <= 0 = []
                | otherwise = x:replicate' (n-1) x


take' :: (Num i, Ord i) => i -> [a] -> [a]
take' _ [] = []
take' n (x:xs)  | n <=0 = []
                | otherwise = x:take' (n-1) xs


reverse' :: [a] -> [a]
reverse' [] = []
reverse' (x:xs) = reverse' xs++[x]


repeat' :: a -> [a]
repeat' x = x:repeat' x


zip' :: [a] -> [b] -> [(a,b)]
zip' [] _ = []
zip' _ [] = []
zip' (x:xs) (y:ys) = (x,y): zip' xs ys



elem' :: (Eq a) => a -> [a] -> Bool
elem' _ [] = False
elem' x (y:ys)  | x == y = True
                | otherwise = elem' x ys


qsort ::  Ord a => [a] -> [a]
qsort [] = []
qsort [x] = [x]
qsort (x:xs) = qsort [z | z <- xs, z <= x] ++ x:qsort [z | z <- xs, z > x]


collatz :: Integer -> [Integer]
collatz 1 = []
collatz n   | even n = n:collatz (div n 2)
            | otherwise = n:collatz (n*3+1)




-- main = print $ rightTriangle 500
