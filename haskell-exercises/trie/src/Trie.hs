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
decode bs t = innerDecode bs t ""
  where
    innerDecode :: Bits -> Trie Char -> String -> String
    innerDecode [] (Leaf c) s = s ++ [c]
    innerDecode (b:bs) (l :-: r) s = if b == T then innerDecode bs r s else innerDecode bs l s
    innerDecode bs (Leaf c) s = [c] ++ innerDecode bs t s


toList::Trie a -> [(a, Bits)]
toList t = inOrder t []
  where
    inOrder::Trie a -> Bits -> [(a, Bits)]
    inOrder (Leaf a) bs = [(a, bs)]
    inOrder (l :-: r) bs = (inOrder l (bs ++ [F])) ++ (inOrder r (bs ++ [T]))
