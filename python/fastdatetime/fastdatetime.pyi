from datetime import datetime


def strptime(date_string: str, format: str) -> datetime: ...

def parse(date_string: str, *, dayfirst: bool = False, yearfirst: bool = False) -> datetime: ...
