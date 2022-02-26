#!venv/bin/python
import logging

from aiogram import Bot, Dispatcher, executor, types
from model.sheet import get_data

# Объект бота
bot = Bot(token="Миші з'їли", parse_mode="MarkdownV2")
# Диспетчер для бота
dp = Dispatcher(bot)
# Включаем логирование, чтобы не пропустить важные сообщения
logging.basicConfig(level=logging.INFO)


@dp.message_handler(commands="example")
async def cmd_test1(message: types.Message):
    text = """
*bold \*text*
_italic \*text_
__underline__
~strikethrough~
||spoiler||
*bold _italic bold ~italic bold strikethrough ||italic bold strikethrough spoiler||~ __underline italic bold___ bold*
[inline URL](http://www.example.com/)
[inline mention of a user](tg://user?id=123456789)
`inline fixed-width code`
```
pre-formatted fixed-width code block
```
```python
pre-formatted fixed-width code block written in the Python programming language
```
    """
    await message.answer(text)

def main():
    # Запуск бота
    executor.start_polling(dp, skip_updates=True)
    

if __name__ == "__main__":
    main()
