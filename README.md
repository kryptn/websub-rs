# Websub

collection of lambda functions to handle maintaining subscriptions to a websub hub and to send notifications to registered consumers.

## Functions

### subscribe

Triggered on insert for subscriptions table

Makes call to websub hub to subscribe to a new topic

### challenge-response:

Triggered via GET request through api gatway. subscription_id is passed through a parameter

verifies known secret and adds a record into the subscription_leases table

### renew

Triggered on ttl expire for subscription_leases table

queries subscriptions table and re-inserts found entry with a new subscribed_at.

this retriggers the subscribe function


### webhook

Triggered via POST request through api gateway. subscription_id is passed through a parameter

todo: verify lease exists for subscription_id

queries subscription_handlers for consumers for this subscription_id

extracts and constructs a notification body

inserts a record into the messages table for each consumer

### notify

Triggered on insert for messages table

queries consumers table for message consumer and sends the message to the consumer


## Plan

- [x] cli add-subscription -> dynamodb insert subscriptions
- [x] on subscriptions insert/update -> subscribe -> send subscribe request
- [x] api_gateway get -> challenge-verify -> dynamodb insert lease | resp 200
- [x] on lease expire -> renew -> dynamodb insert subscription
- [x] cli add-consumer -> add-consumer -> dynamodb insert consumers
- [x] cli create-handler -> create-handler -> dynamodb insert handler
- [x] api_gateway post -> webhook -> find handlers -> for each dynamodb insert message
- [x] on message insert -> notify -> find consumer -> send message
- [ ] on lease delete -> send unsubscribe

## Todos

- write stats function?
- write a way to replay a post request
- write a way to set a state
- write a way to clear state
