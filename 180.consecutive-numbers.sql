/* Write your T-SQL query statement below */
select distinct l1.num as ConsecutiveNums  from Logs l1, Logs l2, Logs l3 where l1.Id = l2.Id - 1
    AND l2.Id = l3.Id - 1
    AND l1.Num = l2.Num
    AND l2.Num = l3.Num;