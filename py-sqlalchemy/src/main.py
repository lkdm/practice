from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from models import Base, User
from typing import List

# an Engine, which the Session will use for connection
# resources
# engine = create_engine("sqlite:///file:database.db?"
#     "check_same_thread=true&timeout=10&nolock=1&uri=true")

def get_users(db: Session) -> List[User]:
    return db.query(User).all()

def main():
    engine = create_engine("sqlite:///:memory:")
    Base.metadata.create_all(engine)
    # create session and add objects
    with Session(engine) as session:
        user = User(name="Joe", fullname="Joe Bloggs", nickname=None)
        session.add(user)
        session.commit()
        users = get_users(session)
        print(users)
        print("Hello from py-sqlalchemy!")

if __name__ == "__main__":
    main()
