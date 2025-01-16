DEFINE FUNCTION OVERWRITE fn::create_wallet($initial_balance: option<float>) {
LET $address = rand::string(10).lowercase();
LET $password = rand::string(16);
LET $hash = crypto::argon2::generate($password);
LET $wallet = (CREATE wallet CONTENT { address: $address, balance: $initial_balance OR 0f, hash: $hash });
RETURN { address: $address, password: $password, wallet: $wallet.first() };
} PERMISSIONS FULL;

DEFINE FUNCTION OVERWRITE fn::transfer_balance($from: record<wallet>, $to: record<wallet> | record<name>, $amount: float) {
UPDATE $from SET balance -= $amount;
UPDATE $from SET total_out += $amount;
RETURN IF record::tb($to) == 'name' {
LET $owner = $to.owner;
UPDATE $owner SET balance += $amount;
UPDATE $owner SET total_in += $amount;
} ELSE {
UPDATE $to SET balance += $amount;
UPDATE $to SET total_in += $amount;
};
} PERMISSIONS FULL;