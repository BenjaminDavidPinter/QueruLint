Declare @datetime datetime = '1/1/2021';
Declare @whatever datetime = '1/1/2021';

begin tran
SELECT * FROM DBO.TABLE AS T1
    INNER JOIN DBO.TABLE2 AS T2 WITH (NOLOCK)
        ON T1.ID = T2.ID