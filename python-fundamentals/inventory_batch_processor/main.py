raw_inventory = [
    ["PHN_IPhone15", 10, 80000],
    ["LAP_MacBookM3", 5, 120000],
    ["ACC_AirPods", 20, 15000],
]
backup_inventory = raw_inventory
while raw_inventory:
    current_batch, raw_inventory = raw_inventory[0], raw_inventory[1:]
    product = current_batch[0]
    catogary = product[:3]
    product = product[4:]
    batch_price = current_batch[1] * current_batch[2]
    print(f"Catogary:{catogary} | Product:{product} | Total Batch Price:{batch_price}")


data_stream = ["Req_1", "Req_2", "Req_3", "Req_4", "Req_5"]
processed_batches = []
while data_stream:
    batch, data_stream = data_stream[0:2], data_stream[2:]
    processed_batches.append(batch)


print(processed_batches)
