from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from data import Base
from data.user import User, get_users
from data.product import Product, get_products, get_product_by_product_code
from data.trade import Trade, Side
from datetime import date

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
    today = date.today()
    trades = [
        Trade(product=products[0], business_date=today, side=Side.B, qty=10),
        Trade(product=products[1], business_date=today, side=Side.S, qty=20)
    ]
    db.add_all(trades)
    db.commit()

def main():
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    # create session and add objects
    with Session(engine) as session:
        seed(session) 

        product = get_product_by_product_code(session, "AUD/USD")
        if product:
            print(product.trades)

        users = get_users(session)
        print(users)
        products = get_products(session)
        print(products)

if __name__ == "__main__":
    main()
