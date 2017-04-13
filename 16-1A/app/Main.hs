module Main where

import System.Environment
import qualified Q1
import qualified Q2
import qualified Q3

main :: IO ()
main = do
    args <- getArgs
    case args of
        "Q1":_ -> Q1.main
        "Q2":_ -> Q2.main
        "Q3":_ -> Q3.main
        _ ->
            putStrLn "Invalid program"
    
