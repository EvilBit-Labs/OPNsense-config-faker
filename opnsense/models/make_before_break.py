from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class MakeBeforeBreak(BaseModel):
    class Meta:
        name = "make_before_break"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
