# import mongodb driver
from pymongo import MongoClient
# import json
import json

class InitializeProducts:
    def __init__(self):
        # connect to mongodb
        self.client = MongoClient('localhost', 27017)
        # connect to database
        self.db = self.client['KitchenManager']
        # connect to collection
        self.collection = self.db['Products']

    def clear(self):
        # delete all data from collection
        self.collection.delete_many({})
        # reset auto increment
        self.collection.drop_indexes()
        self.collection.create_index("id", unique=True, dropDups=True)

    def insert(self):
        # open json file
        with open('products.json') as f:
            # load json file
            file_data = json.load(f)
        # insert json file data into mongodb
        self.collection.insert(file_data)

if __name__ == '__main__':
    # initialize class
    init = InitializeProducts()
    # clear collection
    init.clear()
    # insert data
    init.insert()
