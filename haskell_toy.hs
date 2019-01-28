
factorial :: Integer -> Integer
factorial n = product [1..n]

pythagorasTriples :: Int -> [(Int, Int, Int)]
pythagorasTriples sideSum = [(a,b,c) | c <- [1..div sideSum 2],
                                       b <- [1..c],
                                       a <- [1..b],
                                a+b+c == sideSum, a^2+b^2==c^2]

rightTriangle :: Int -> [(Int, [(Int, Int, Int)])]
rightTriangle sideSum = [v | v <- [(x, pythagorasTriples x) | x <- [1..sideSum]], not (null (snd v))]

circunference :: Float -> Float
circunference r = 2*pi*r

circunference' :: Double -> Double
circunference' r = 2*pi*r

