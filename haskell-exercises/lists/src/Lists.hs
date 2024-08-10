module Lists (member, union, intersection, difference,
              insert, insertionSort,
              binaryToDecimal, toDecimal, toDec, decimal,
              binaryAdd) where

import Data.Char(digitToInt)

member:: Int -> [Int] -> Bool
member _ []      = False
member e (x:xs)  = e == x || member e xs


union:: [Int] -> [Int] -> [Int]
union [] ys     = ys
union (x:xs) ys
  | member x ys = union xs ys
  | otherwise   = x : union xs ys

-- Remove Implementations, from, here on

intersection:: [Int] -> [Int] -> [Int]
intersection [] ys = []
intersection (x:xs) ys
  | member x ys = x : intersection xs ys
  | otherwise = intersection xs ys

difference:: [Int] -> [Int] -> [Int]
difference [] ys = []
difference xs [] = xs
difference (x:xs) ys
  | not (member x ys) = x : difference xs ys
  | otherwise = difference xs ys

insert:: Int -> [Int] -> [Int]
insert e [] = e : []
insert e (x:xs)
  | e > x = x : insert e xs
  | otherwise = e : (x:xs)

insertionSort :: [Int] -> [Int]
insertionSort [] = []
--insertionSort (x:xs) = insert x (insertionSort xs)
insertionSort xs = foldr insert [] xs

binaryToDecimal :: [Int] -> Int
binaryToDecimal xs = sum [x * (2 ^ (size - i)) | (x, i) <- zip xs [1..size]]
  where size = length(xs)

toDecimal :: Int -> [Int] -> Int
toDecimal r xs = sum [x * (r ^ (size - i)) | (x, i) <- zip xs [1..size]]
  where size = length(xs)

toDec::Int -> String -> Int
toDec base s = error "Implement it"

-- Same as `toDec` But use a list comprehension

decimal::Int -> String -> Int
decimal  = error "Implement it"

firsts::[a] -> [[a]]
firsts = error "Implement it"

-- Given two String that represents numbers in binary implement the 'binaryAdd' function
-- DO NOT USE a predefined '+' operation

binaryAdd::String -> String -> String
binaryAdd  = error "Implement it"