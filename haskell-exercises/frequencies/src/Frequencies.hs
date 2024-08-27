module Frequencies  (Frequency, frequencyMap, frequencies, insert, insertionSort) where

import Data.Map(Map)
import qualified Data.Map as Map
import Data.Tuple(swap)

type Frequency = (Int, Char)

frequencies::String -> [Frequency]
frequencies s = frequenciesList
  where
    frequenciesMap = frequencyMap s
    frequenciesTuples = Map.toList frequenciesMap
    frequenciesList = map swap frequenciesTuples

frequencyMap::(Ord a) => [a] -> Map a Int
frequencyMap as = innerMapping as Map.empty
  where
    innerMapping::(Ord a) => [a] -> Map a Int -> Map a Int
    innerMapping [] map = map
    innerMapping (c:cs) map = innerMapping cs updatedMap
      where updatedMap = Map.insertWith (+) c 1 map

insert::(Ord a) => a -> [a] -> [a]
insert a [] = a : []
insert a (x:xs)
  | a > x = x : insert a xs
  | otherwise = a : (x:xs)

insertionSort :: (Ord a) => [a] -> [a]
insertionSort [] = []
insertionSort xs = foldr insert [] xs
