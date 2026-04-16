from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from sqlalchemy import Integer, String, ForeignKey
from sqlalchemy.orm import Session
from data import Base
from data.product import Product

class Trade(Base):
    __tablename__ = "trade"
    id: Mapped[int] = mapped_column(primary_key=True, index=True, autoincrement=True)
    product_id: Mapped[int] = mapped_column(ForeignKey(Product.id))

    product: Mapped[List["Product"]] = relationship(back_populates="trades")

def get_trades(db: Session) -> List[Trade]:
    return db.query(Trade).all()
