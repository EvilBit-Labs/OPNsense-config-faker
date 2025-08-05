from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Category2(BaseModel):
    class Meta:
        name = "category"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
