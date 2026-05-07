from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from shared import Base
from trading import run as run_trades
from landlord import run as run_landlord


def main():
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    # create session and add objects
    with Session(engine) as db:
        # run_trades(db)
        run_landlord(db)

if __name__ == "__main__":
    main()
