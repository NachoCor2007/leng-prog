module CollatzConjecture (collatz) where

collatz :: Integer -> Maybe Integer
collatz number
  | number <= 0 = Nothing
  | otherwise = Just (recursiveCollatz number 0)

recursiveCollatz :: Integer -> Integer -> Integer
recursiveCollatz number iteration
  | number == 1 = iteration
  | even number = recursiveCollatz half nextIteration
  | otherwise = recursiveCollatz nextNum nextIteration
    where
      half = number `div` 2
      nextNum = 3 * number + 1
      nextIteration = iteration + 1
