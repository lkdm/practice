from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from data import Base
from data.user import get_users
from data.product import get_products, get_product_by_product_code

from trades import run as run_trades

def main():
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    # create session and add objects
    with Session(engine) as db:
        run_trades(db)

if __name__ == "__main__":
    main()
