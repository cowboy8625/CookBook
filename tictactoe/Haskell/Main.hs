module Main where
import Control.Monad (unless)
import Data.Char

main :: IO ()
main = do
  putStrLn $ show input


input :: Int
input = do
    c <- getChar
    if not $ isDigit c then
        input
    else
        read c
