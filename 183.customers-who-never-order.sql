 /* Write your T-SQL query statement below */
SELECT  Name AS CustomersFROM Customers AS c
    WHERE Not EXISTS ( 
        SELECT  *
        FROM Orders AS o
        WHERE c.Id = o.CustomerId) 