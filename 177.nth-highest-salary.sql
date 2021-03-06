CREATE FUNCTION getNthHighestSalary(@N INT) RETURNS INT AS
BEGIN
    RETURN (
        SELECT  top 1 salary
        FROM 
        (
            SELECT  salary
                   ,dense_rank() over(order by salary desc) rn
            FROM employee
        ) a
        WHERE rn=@N 
    );
END