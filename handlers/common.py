from aiogram import Dispatcher, types
from aiogram.dispatcher.storage import FSMContext


async def cmd_start(message: types.Message, state: FSMContext):
    await state.finish()
    await message.answer(
        "Виберіть, що ви хочете\.", reply_markup=types.ReplyKeyboardRemove()
    )


async def cmd_cancel(message: types.Message, state: FSMContext):
    await state.finish()
    await message.answer("Дію скасовано.", reply_markup=types.ReplyKeyboardRemove())


def register_handlers_common(dp: Dispatcher):
    dp.register_message_handler(cmd_start, commands="start", state="*")
    dp.register_message_handler(cmd_cancel, commands="cancel", state="*")
