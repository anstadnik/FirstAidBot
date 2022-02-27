from typing import Callable

import aiogram.utils.markdown as fmt
from aiogram import Dispatcher, types
from aiogram.dispatcher import FSMContext
from aiogram.dispatcher.filters.state import State, StatesGroup
from model.sheet import FiniteState, FiniteStateOptions, OptionalUrlAndStr


class FirstAid(StatesGroup):
    dialog = State()


def gen_keyboard(buttons: list[OptionalUrlAndStr]):
    keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
    for keys in zip(buttons[::2], buttons[1::2]):
        keyboard.add(*keys)
    if len(buttons) % 2:
        keyboard.add(buttons[-1])
    keyboard.add("◀️На початок")
    return keyboard


async def setup(message: types.Message, state: FSMContext, data: FiniteState):
    try:
        msg, possible_options = data
    except Exception as e:
        await state.finish()
        await message.answer(
            "Це ви щось погане зробили ваще, кличте програмістів",
            reply_markup=types.ReplyKeyboardRemove(),
        )
        await message.answer(str(e), parse_mode=types.ParseMode.MARKDOWN)
        return

    assert possible_options is not None

    await state.set_data(possible_options)
    keyboard = gen_keyboard(list(possible_options.keys()))
    if isinstance(msg, tuple):
        link, msg = msg
        await message.answer(fmt.hide_link(link), parse_mode=types.ParseMode.HTML)

    try:
        await message.answer(msg, reply_markup=keyboard)
    except Exception as e:
        await message.answer(str(e), parse_mode=types.ParseMode.MARKDOWN)

    await FirstAid.dialog.set()


async def process_input(message: types.Message, state: FSMContext, data: FiniteState):
    inp = message.text

    if inp == "На початок":
        await message.answer(
            r"Повертаємось\. Дякую, що допомагаєте, ви сонечко\! 🐞",
            reply_markup=types.ReplyKeyboardRemove(),
        )
        await setup(message, state, data)

    if inp not in (await state.get_data()).keys():
        await message.answer("Wrong, use buttons please")
        return

    msg: str
    possible_options: FiniteStateOptions
    msg, possible_options = (await state.get_data())[inp]

    if possible_options is None:
        try:
            await message.answer(msg)
        except Exception as e:
            await state.finish()
            keyboard = types.ReplyKeyboardRemove()
            await message.answer(
                str(e), reply_markup=keyboard, parse_mode=types.ParseMode.MARKDOWN
            )
            return await setup(message, state, data)
        return await setup(message, state, data)

    keyboard = gen_keyboard(list(possible_options.keys()))
    await state.update_data(possible_options)

    if isinstance(msg, tuple):
        link, msg = msg
        await message.answer(fmt.hide_link(link), parse_mode=types.ParseMode.HTML)

    try:
        await message.answer(msg, reply_markup=keyboard)
    except Exception as e:
        await state.finish()
        keyboard = types.ReplyKeyboardRemove()
        await message.answer(
            str(e), reply_markup=keyboard, parse_mode=types.ParseMode.MARKDOWN
        )
        return await setup(message, state, data)


def get_handler(data: FiniteState, reset=False):
    async def handler(message: types.Message, state: FSMContext):
        if reset:
            await state.finish()
            await FirstAid.dialog.set()
        if not await state.get_data():
            return await setup(message, state, data)

        return await process_input(message, state, data)

    return handler


def register_handlers_first_aid(dp: Dispatcher, data: FiniteState):
    dp.register_message_handler(get_handler(data, True), commands="start", state="*")
    dp.register_message_handler(get_handler(data, False), state=FirstAid.dialog)
