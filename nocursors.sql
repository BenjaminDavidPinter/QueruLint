DECLARE @name VARCHAR(20)  = "BEN" --SOME COMMENT HERE
DECLARE @LASTNAME varchar(20) = "Pinter" -- some other COMMENT

DECLARE dbcurse CURSOR FOR 
SELECT name
FROM DBO.TABLETHING

open dbcurse
fetch next from dbcurse into @name

while @FETCH_STATUS = 0
BEGIN
	//do stuff
end
close dbcurse
deallocate dbcurse

