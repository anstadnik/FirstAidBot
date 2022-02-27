import aiogram.utils.markdown as fmt
from aiogram import Dispatcher, types
from aiogram.dispatcher import FSMContext
from aiogram.dispatcher.filters.state import State, StatesGroup
from model.sheet import FiniteState, FiniteStateOptions, get_data


# I'm ashamed
class FirstAid(StatesGroup):
    start = State()
    dialog = State()


def get_handler_start(data: FiniteState):
    async def first_aid_handler(message: types.Message, state: FSMContext):
        try:
            msg, possible_options = get_data()
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
        if isinstance(msg, tuple):
            link, msg = msg
            await message.answer(fmt.hide_link(link), parse_mode=types.ParseMode.HTML)
        await message.answer(msg, reply_markup=keyboard)
        await FirstAid.dialog.set()

    return first_aid_handler


async def first_aid_handler(message: types.Message, state: FSMContext):
    inp = message.text
    if inp not in (await state.get_data()).keys():
        await message.answer("Wrong, use buttons please")
        return

    msg: str
    possible_options: FiniteStateOptions
    msg, possible_options = (await state.get_data())[inp]

    if possible_options is None:
        await state.finish()
        keyboard = types.ReplyKeyboardRemove()
    else:
        keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
        for name in possible_options.keys():
            keyboard.add(name)
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


def register_handlers_first_aid(dp: Dispatcher, data: FiniteState):
    dp.register_message_handler(
        get_handler_start(data), commands="first_aid", state="*"
    )
    dp.register_message_handler(first_aid_handler, state=FirstAid.start)
    dp.register_message_handler(first_aid_handler, state=FirstAid.dialog)
