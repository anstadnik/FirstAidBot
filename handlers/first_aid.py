from aiogram import Dispatcher, types
from aiogram.dispatcher import FSMContext
from aiogram.dispatcher.filters.state import State, StatesGroup
from model.sheet import FiniteState


# I'm ashamed
class FirstAid(StatesGroup):
    start = State()
    dialog = State()


def get_handler_start(data: FiniteState):
    async def first_aid_handler(message: types.Message, state: FSMContext):
        question, possible_options = data
        assert possible_options is not None

        await state.set_data(possible_options)
        keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
        for name in possible_options.keys():
            keyboard.add(name)
        await message.answer(question, reply_markup=keyboard)
        await FirstAid.next()

    return first_aid_handler


async def first_aid_handler(message: types.Message, state: FSMContext):
    possible_options: dict[str, FiniteState] = await state.get_data()
    inp = message.text.lower()
    if inp not in possible_options.keys():
        await message.answer("Wrong, use buttons please")
        return

    msg, possible_options = possible_options[inp]
    keyboard = None
    if next is None:
        await FirstAid.first()
        return

    msg, possible_options = next
    keyboard = types.ReplyKeyboardMarkup(resize_keyboard=True)
    for name in possible_options.keys():
        keyboard.add(name)
    await state.update_data(possible_options)

    await message.answer(msg, reply_markup=keyboard)


def register_handlers_first_aid(dp: Dispatcher, data: FiniteState):
    dp.register_message_handler(
        get_handler_start(data), commands="first_aid", state="*"
    )

    dp.register_message_handler(first_aid_handler, state=FirstAid.dialog)
