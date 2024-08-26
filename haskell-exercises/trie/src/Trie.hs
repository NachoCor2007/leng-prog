module Trie  (Trie(..), left, right, find, decode, toList) where

import Bit

data Trie a = Leaf a | Trie a :-: Trie a deriving (Eq, Show)

left::(Show a) => Trie a -> Trie a
left (Leaf a) = error $ "Left of: " ++ (show (Leaf a))
left (l :-: _) = l

right::(Show a) => Trie a -> Trie a
right (Leaf a) = error $ "Right of: " ++ (show (Leaf a))
right (_ :-: r) = r

find::Bits -> Trie a -> a
find [] (Leaf a) = a
find [] (_ :-: _) = error "No Leaf found"
find (b:bs) (l :-: r) = if b == T then find bs r else find bs l

decode::Bits -> Trie Char -> String
decode = error "Define it"

toList::Trie a -> [(a, Bits)]
toList = error "Define it"
