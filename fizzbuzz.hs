fizzbuzz :: (Show t, Integral t) => t -> t -> t -> IO ()
fizzbuzz fizznum buzznum x
    | both = print "FizzBuzz"
    | fizz = print "Fizz"
    | buzz = print "Buzz"
    | otherwise = print x
    where fizz = mod x fizznum == 0
          buzz = mod x buzznum == 0
          both = fizz && buzz

main = mapM_ (fizzbuzz 3 5) [1..200]
