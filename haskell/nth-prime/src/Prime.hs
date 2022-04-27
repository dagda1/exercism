module Prime (isPrime, nth) where

isPrime :: Int -> Bool
isPrime n
  | n < 2          = False
  | n == 2         = True
  | n `mod` 2 == 0 = False
  | otherwise = not $ any (\x -> n `mod` x == 0) $ tail [1, 3..(ceiling . sqrt . fromIntegral) n]

nthPrime :: Int -> Int -> Int -> Int -> Int
nthPrime n count currentCount currentPrime
  | n == 1                  = 2
  | count > 110000          = -1
  | currentCount > n        = -1
  | currentCount == n       = currentPrime
  | otherwise               = if (isPrime count) == True then 
                                nthPrime n (count + 1) (currentCount + 1) count
                              else
                                nthPrime n (count + 1) currentCount currentPrime


                          
nth :: Int -> Int
nth n = nthPrime n 3 1 2
