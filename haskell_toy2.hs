import Data.Char
import qualified Data.Map as Map

somaChar :: (Integral a, Show a) => a -> Int
-- somaChar num = sum [ord x - 48 | x <- show num]
somaChar = sum . map digitToInt . show


catal :: Map.Map String String
catal = Map.fromList $ [("a", "1")]

--e :: Integer -> Float ->  Float
e :: (Floating a) => Integer -> a ->  a
e 0 _ = 1
e n x = ((x^n)/fromIntegral (product [1..n] )) + e (n-1) x
