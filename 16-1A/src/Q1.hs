module Q1 (
    main
    ) where

import Data.Char as Char
import Control.Monad 

solve :: String -> String
solve str =
    foldl solve' [] str
    where
      solve' [] c = [c]
      solve' s@(x:_) c
          | c < x = s ++ [c]
          | otherwise = c : s
              
main :: IO()
main = do
    t <- getInt
    forM_ [1..t] $ \i -> do
        [w] <- getWords 1
        printCase i
        putStrLn $ solve w
    

takeWhileM :: (Monad m) => (a -> Bool) -> [m a] -> m [a]
takeWhileM _ [] = return []
takeWhileM p (x:xs) = do
    v <- x
    if p v
        then (v : ) <$> takeWhileM p xs
        else return []

getWords :: Int -> IO [String]
getWords x
    | x > 0 = do
          word <- takeWhileM (not . isSpace) $ repeat getChar
          (word :) <$> getWords (x-1)
    | otherwise = return []

getInts :: Int -> IO [Integer]
getInts x = map read <$> getWords x

getInt :: IO Integer
getInt = head <$> getInts 1

printCase :: Integer -> IO ()
printCase x = do
    putStr $ "Case #" ++ (show x) ++ ": "
    
