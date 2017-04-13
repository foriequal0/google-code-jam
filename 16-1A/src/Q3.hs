module Q3 (
    main
    ) where

import Data.Char as Char
import Control.Monad
import Data.List
import Data.Maybe
import Control.Arrow
import qualified Data.Set as Set {-- require containers >= 0.5.6 && < 0.6--}

infix 0 $> 
($>) :: a -> (a -> b) -> b
x $> f = f x

solve :: [Int] -> String
solve bffs =
    let childs = [1..length bffs]
        directed = zip childs bffs
        reverse = map (\(f, s) -> (s, f)) directed
        graph = (reverse ++ directed)
            $> sortBy (\x y-> compare (fst x) (fst y))
            >>> nub
            >>> Set.fromList
    in show
       . maximumBy (\x y -> compare (length . fst $ x) (length . fst $ y))
       . concatMap (filter (\x -> snd x == [])
                    . dfs (\r e -> e:r) [] graph)
       $ [1..length bffs]

main :: IO()
main = do
    t <- getInt
    forM_ [1..t] $ \i -> do
        n <- getInt
        bffs <- getInts n
        printCase i
        putStrLn $ solve bffs

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

getInts :: Int -> IO [Int]
getInts x = map read <$> getWords x

getInt :: IO Int
getInt = head <$> getInts 1

printCase :: Int -> IO ()
printCase x = do
    putStr $ "Case #" ++ (show x) ++ ": "
    
