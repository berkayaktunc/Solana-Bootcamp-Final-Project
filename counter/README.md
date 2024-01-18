# COUNTER

In this homework, you will modify the transfer amount in out existing logic. Currently it is transferring all the balance source account holds. Instead let's change this to a defined amount.

### Step 1

Modify this line: let amount = source_account.amount so that it will transfer a hard coded amount.

### Step 2

Run cargo test to make sure project is still working

### Optional

Instead of hard coding the amount, you can get the amount as input from the user and then transfer that amount of tokens.


# How to run?

Clone the repo
```bash
git clone <link>
```

Get in the folder
```bash
cd Solana-Bootcamp-Final-Project/counter
```

Build or test
```bash
make build    # how you build
make test     # how you test
make testcap  # how you test with nocapture
```