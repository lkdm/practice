from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from data import Base
from data.user import User, get_users
from data.product import Product, get_products
from data.trade import Trade
from typing import List, Any

def seed(db: Session):

    users = [
        User(name="Joe", fullname="Joe Bloggs", nickname=None)
    ]
    db.add_all(users)
    products = [
        Product(product_code="AUD/USD"),
        Product(product_code="XAU/USD"),
        Product(product_code="NZD/USD"),
        Product(product_code="JPY/USD"),
    ]
    db.add_all(products)
    trades = [
        Trade(product=products[0]),
        Trade(product=products[1])
    ]
    db.add_all(trades)
    db.commit()

def main():
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    # create session and add objects
    with Session(engine) as session:
        seed(session) 


        users = get_users(session)
        print(users)
        products = get_products(session)
        print(products)

if __name__ == "__main__":
    main()
