DEFINE EVENT OVERWRITE transfer_balance ON transaction WHEN $event = 'CREATE' THEN {
LET $from = $after.from;
LET $to = $after.to;
LET $amount = $after.amount;
RETURN fn::transfer_balance($from, $to, $amount);
};