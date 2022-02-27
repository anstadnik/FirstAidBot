import aiogram.utils.markdown as fmt
from aiogram import Dispatcher, types
from aiogram.dispatcher import FSMContext
from aiogram.dispatcher.filters.state import State, StatesGroup
from model.sheet import FiniteState, FiniteStateOptions


class FirstAid(StatesGroup):
    start = State()
    dialog = State()


def get_handler_start(data: FiniteState):
    async def first_aid_handler(message: types.Message, state: FSMContext):
        try:
            msg, possible_options = data
        except Exception as e:
            await state.finish()
            keyboard = types.ReplyKeyboardRemove()
            await message.answer(
                "Це ви щось погане зробили ваще, кличте програмістів",
                reply_markup=keyboard,
            )
            await message.answer(
                str(e), reply_markup=keyboard, parse_mode=types.ParseMode.MARKDOWN
            )
            return

        assert possible_options is not None

        await state.set_data(possible_options)
        keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
        for name in possible_options.keys():
            keyboard.add(name)
        keyboard.add("На початок")
        if isinstance(msg, tuple):
            link, msg = msg
            await message.answer(fmt.hide_link(link), parse_mode=types.ParseMode.HTML)
        await message.answer(msg, reply_markup=keyboard)
        await FirstAid.dialog.set()

    return first_aid_handler


async def first_aid_handler(message: types.Message, state: FSMContext):
    inp = message.text
    if inp == "На початок":
        await state.finish()
        await message.answer(
            r"Повертаємось\. Дякую, що допомагаєте, ви сонечко\! 🐞",
            reply_markup=types.ReplyKeyboardRemove(),
        )
        await FirstAid.start.set()
        return
    if inp not in (await state.get_data()).keys():
        await message.answer("Wrong, use buttons please")
        return

    msg: str
    possible_options: FiniteStateOptions
    msg, possible_options = (await state.get_data())[inp]

    if possible_options is None:
        await state.finish()
        keyboard = types.ReplyKeyboardRemove()
        await FirstAid.start.set()
    else:
        keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
        for name in possible_options.keys():
            keyboard.add(name)
        keyboard.add("На початок")
        await state.update_data(possible_options)

    try:
        if isinstance(msg, tuple):
            link, msg = msg
            await message.answer(fmt.hide_link(link), parse_mode=types.ParseMode.HTML)
        await message.answer(msg, reply_markup=keyboard)
    except Exception as e:
        await state.finish()
        keyboard = types.ReplyKeyboardRemove()
        await message.answer(
            str(e), reply_markup=keyboard, parse_mode=types.ParseMode.MARKDOWN
        )
        await FirstAid.start.set()


def register_handlers_first_aid(dp: Dispatcher, data: FiniteState):
    start_handler = get_handler_start(data)
    dp.register_message_handler(start_handler, commands="start", state="*")
    dp.register_message_handler(start_handler, state=FirstAid.start)
    dp.register_message_handler(first_aid_handler, state=FirstAid.dialog)
