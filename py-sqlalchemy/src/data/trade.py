from sqlalchemy.orm import Mapped
from sqlalchemy.orm import mapped_column, relationship
from typing import Optional, List
from enum import Enum
from sqlalchemy import Integer, String, ForeignKey, Date
from sqlalchemy import Enum as SAEnum
from sqlalchemy.orm import Session
from data import Base
from data.product import Product
from datetime import date

class Side(str, Enum):
    B = "B"
    S = "S"

class Trade(Base):
    __tablename__ = "trade"
    id: Mapped[int] = mapped_column(primary_key=True, index=True, autoincrement=True)
    product_id: Mapped[int] = mapped_column(ForeignKey(Product.id))
    business_date: Mapped[date] = mapped_column(Date, nullable=False)
    side: Mapped[Side] = mapped_column(SAEnum(Side), nullable=False)
    qty: Mapped[int] = mapped_column(Integer, nullable=False)

    product: Mapped[List["Product"]] = relationship(back_populates="trades")

    def __repr__(self) -> str:
        return f"Trade(id={self.id}, product_id={self.product_id}, business_date={self.business_date}, qty={self.qty})"

    def __str__(self) -> str:
        return f"{self.business_date} {self.side} {self.product} {self.qty}"

def get_trades(db: Session) -> List[Trade]:
    return db.query(Trade).all()
