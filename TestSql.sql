Declare @datetime datetime = '1/1/2021';
Declare @whatever datetime = '2/4/2021';

begin tran
SELECT * FROM DBO.TABLE AS T1
    INNER JOIN DBO.TABLE2 AS T2 WITH (NOLOCK)
        ON T1.ID = T2.ID
where T1.createddt > GetDate()
or somefunc() = 'ur mum'

declare @badidea int = 0;

declare @anotherBadIdea int
end tran