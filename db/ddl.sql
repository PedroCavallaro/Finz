CREATE TABLE Goals(
    "id" UUID NOT NULL,
    "description" VARCHAR(255) NOT NULL,
    "balance" DOUBLE PRECISION NOT NULL,
    "goal_amount" DOUBLE PRECISION NOT NULL
);
ALTER TABLE
    Goals ADD PRIMARY KEY("id");
CREATE TABLE Transaction_type(
    "id" UUID NOT NULL,
    "type" VARCHAR(255) NOT NULL
);
ALTER TABLE
    Transaction_type ADD PRIMARY KEY("id");
CREATE TABLE Users(
    "id" UUID NOT NULL,
    "name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "picture" VARCHAR(255) NOT NULL
);
ALTER TABLE
    Users ADD PRIMARY KEY("id");
CREATE TABLE Transactions(
    "id" UUID NOT NULL,
    "amount" FLOAT NOT NULL,
    "description" UUID NOT NULL,
    "date" DATE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "transaction_type" UUID NOT NULL,
    "account_id" UUID NOT NULL
);
ALTER TABLE
    Transactions ADD PRIMARY KEY("id");


CREATE TABLE Account(
    "id" UUID NOT NULL,
    "balance" DOUBLE PRECISION NOT NULL DEFAULT 0,
    "currency_type" VARCHAR(255) NOT NULL DEFAULT '$',
    "provider_id" VARCHAR(255) NOT NULL,
    "created_at" DATE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
ALTER TABLE
    Account ADD PRIMARY KEY("id");
-- CREATE TABLE "Plan"(
--     "id" UUID NOT NULL,
--     "plan" geography(LINESTRING, 4326) NOT NULL,
--     "fee_amount" DOUBLE PRECISION NOT NULL,
--     "description" VARCHAR(255) NOT NULL
-- );
-- ALTER TABLE
--     "Plan" ADD PRIMARY KEY("id");
-- CREATE TABLE "Fee"(
--     "id" UUID NOT NULL,
--     "started_at" DATE NOT NULL,
--     "last charge" DATE NOT NULL,
--     "account_id" UUID NOT NULL,
--     "plan_id" UUID NOT NULL,
--     "free_trial" BOOLEAN NOT NULL
-- );
-- ALTER TABLE
--     "Fee" ADD PRIMARY KEY("id");
-- CREATE TABLE "Currency"(
--     "id" UUID NOT NULL,
--     "currency_type" VARCHAR(255) NOT NULL
-- );
-- ALTER TABLE
--     "Currency" ADD PRIMARY KEY("id");
-- ALTER TABLE
--     "Fee" ADD CONSTRAINT "fee_plan_id_foreign" FOREIGN KEY("plan_id") REFERENCES "Plan"("id");
-- ALTER TABLE
--     "Currency" ADD CONSTRAINT "currency_currency_type_foreign" FOREIGN KEY("currency_type") REFERENCES "Account"("id");
ALTER TABLE
    Transactions ADD CONSTRAINT "transactions_transaction_type_foreign" FOREIGN KEY("transaction_type") REFERENCES Transaction_type("id");
-- ALTER TABLE
--     "Fee" ADD CONSTRAINT "fee_id_foreign" FOREIGN KEY("id") REFERENCES "Account"("id");
ALTER TABLE
    Transactions ADD CONSTRAINT "transactions_account_id_foreign" FOREIGN KEY("account_id") REFERENCES Account("id");
ALTER TABLE
    Goals ADD CONSTRAINT "goals_id_foreign" FOREIGN KEY("id") REFERENCES Account("id");
ALTER TABLE
    Account ADD CONSTRAINT "account_id_foreign" FOREIGN KEY("id") REFERENCES Users("id");