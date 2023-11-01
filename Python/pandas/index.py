import pandas as pd
import asyncio

async def main():
    for i in range(0, 10000):
        pd.read_excel('testfile.xlsx')
        print(i)
asyncio.run(main())