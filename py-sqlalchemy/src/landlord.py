from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from data import Base
from data.user import User, get_users
from data.product import Product, get_products, get_product_by_product_code
from data.trade import Trade, Side
from datetime import date, timedelta

def seed(db: Session):

