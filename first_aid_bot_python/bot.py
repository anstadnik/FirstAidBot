#!venv/bin/python
import asyncio
import logging

from aiogram import Bot, Dispatcher
from aiogram.contrib.fsm_storage.memory import MemoryStorage
from aiogram.types.bot_command import BotCommand

from handlers.first_aid import register_handlers_first_aid
from model.sheet import get_data


# Регистрация команд, отображаемых в интерфейсе Telegram
async def set_commands(bot: Bot):
    commands = [BotCommand(command="/start", description="Почати роботу")]
    await bot.set_my_commands(commands)


async def main():
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(name)s - %(message)s",
    )

    logger = logging.getLogger(__name__)
    logger.debug("Starting bot")

    bot = Bot(
        token="5253437681:AAES1RtIBXBanwaWvFk3vWaty-_LH1xYge0", parse_mode="MarkdownV2"
    )
    dp = Dispatcher(bot, storage=MemoryStorage())
    logging.basicConfig(level=logging.INFO)

    data = get_data()

    # Регистрация хэндлеров
    register_handlers_first_aid(dp, data)

    await set_commands(bot)

    await dp.start_polling()


if __name__ == "__main__":
    asyncio.run(main())
