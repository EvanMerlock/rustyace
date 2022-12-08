use crate::types::*;
use ace_gl_types::gl;
use std::rc::Rc;
use tobj;
use std::path::Path;
use nalgebra;

pub struct ObjModel {
    vertices: nalgebra::DMatrix<f32>,
    indices: Vec<u32>,
    material: Option<String>,

    vao: VertexArrayObj,
    vbo: VertexBufferObj,
    eao: ElementArrayObj,

    shader: CompiledShaderProgram,
}

impl ObjModel {
    /// Loads all of the models and materials associated with one object file
    pub fn from_file<S: AsRef<Path> + ::std::fmt::Debug> (gl_ctx: Rc<gl::Gl>, loc: S) -> Result<(Vec<ObjModel>, Vec<Material>), tobj::LoadError> {

        let (models, materials) = tobj::load_obj(loc, true)?;

        let mut model_result: Vec<ObjModel> = Vec::new();
        let material_result: Vec<Material> = materials.into_iter().map(|mat| Material::from(mat)).collect();

        // first we iterate over all of the models, then all of the materials
        // maybe should be backwards, so that we can reference the material name for the model
        for model in models {
            let mesh = model.mesh;
            let mat_name = mesh.material_id.and_then(|id| material_result.get(id)).map(|x| x.name.clone());
            // big branch here; generate an object model based on the vertex data provided by the mesh
            if mesh.normals.len() == 0 && mesh.texcoords.len() == 0 {
                model_result.push(ObjModel::from_mesh(gl_ctx.clone(), mesh, mat_name.clone()));
            } else if mesh.normals.len() == 0 {
                model_result.push(ObjModel::from_mesh_tex(gl_ctx.clone(), mesh, mat_name.clone()));
            } else if mesh.texcoords.len() == 0 {
                model_result.push(ObjModel::from_mesh_norm(gl_ctx.clone(), mesh, mat_name.clone()));
            } else {
                model_result.push(ObjModel::from_mesh_texnorm(gl_ctx.clone(), mesh, mat_name.clone()));
            }
            
        }

        Ok((model_result, material_result))
    }

    fn from_mesh_texnorm(gl_ctx: Rc<gl::Gl>, mesh: tobj::Mesh, mat_name: Option<String>) -> ObjModel {
        let num_rows = mesh.positions.len() / 3;
        let internal_matrix = nalgebra::DMatrix::from_column_slice(3, num_rows, &mesh.positions);
        let tex_matrix = nalgebra::DMatrix::from_column_slice(2, num_rows, &mesh.texcoords);
        let norm_matrix = nalgebra::DMatrix::from_column_slice(3, num_rows, &mesh.normals);

        let internal_matrix = augment_bottom(internal_matrix, norm_matrix)
            .map(|mat| augment_bottom(mat, tex_matrix)).expect("dims did not match");

        unimplemented!()
    }

    fn from_mesh_tex(gl_ctx: Rc<gl::Gl>, mesh: tobj::Mesh, mat_name: Option<String>) -> ObjModel {
        unimplemented!()
    }

    fn from_mesh_norm(gl_ctx: Rc<gl::Gl>, mesh: tobj::Mesh, mat_name: Option<String>) -> ObjModel {
        unimplemented!()
    }

    fn from_mesh(gl_ctx: Rc<gl::Gl>, mesh: tobj::Mesh, mat_name: Option<String>) -> ObjModel {
        unimplemented!()
    }
}

impl Model for ObjModel {
    fn get_vertices(&self) -> &Vec<f32> {
        unimplemented!()
    }

    fn get_indices(&self) -> &Vec<u32> {
        unimplemented!()
    }

    fn get_vert_array_obj(&self) -> &VertexArrayObj {
        unimplemented!()
    }

    fn get_vert_buffer_obj(&self) -> &VertexBufferObj {
        unimplemented!()
    }

    fn get_elem_array_obj(&self) -> &ElementArrayObj {
        unimplemented!()
    }

    fn get_shader(&self) -> &Rc<CompiledShaderProgram> {
        unimplemented!()
    }
}

fn augment_right<N: nalgebra::Scalar + Copy>(me: nalgebra::DMatrix<N>, other: nalgebra::DMatrix<N>) -> Option<nalgebra::DMatrix<N>> {
    // augmenting to the right (number of rows is constant, adding new columns)
    let (num_rows, num_columns) = me.shape();

    if num_rows == 0 || num_columns == 0 {
        return None;
    }

    let (aug_rows, aug_columns) = other.shape();
    if num_rows != aug_rows {
        return None;
    }

    // insert new columns to place stuff into
    let insert_val_temp = *me.get((0,0)).expect("totally empty matrix passed into augment and further than initial check, big rip");
    let mut me = me.insert_columns(num_rows, aug_columns, insert_val_temp);

    for column_index in num_columns..aug_columns+num_columns {
        // normalize to other
        let aug_column_index = column_index - num_columns;
        let column_slice = other.column(aug_column_index);
        for row_index in 0..num_rows {
            me[(row_index, column_index)] = column_slice[row_index];
        }

    }

    Some(me)

}

fn augment_bottom<N: nalgebra::Scalar + Copy>(me: nalgebra::DMatrix<N>, other: nalgebra::DMatrix<N>) -> Option<nalgebra::DMatrix<N>> {
    // augmenting to the bottom (number of rows changes, number of columns is consistent)
    let (num_rows, num_columns) = me.shape();

    if num_rows == 0 || num_rows == 0 {
        return None;
    }

    let (aug_rows, aug_columns) = other.shape();
    if num_columns != aug_columns {
        return None;
    }

    let insert_val_temp = *me.get((0, 0)).expect("totally empty matrix passed into augment_bottom and further than initial check, big rip");
    let mut me = me.insert_rows(num_rows, aug_rows, insert_val_temp);

    for row_index in num_rows..num_rows+aug_rows {
        let aug_row_index = row_index - num_rows;
        let row_slice = other.row(aug_row_index);
        for column_index in 0..num_columns {
            me[(row_index, column_index)] = row_slice[column_index];
        }
    }

    Some(me)
}

#[cfg(test)]
mod tests {
    #[test]
    fn augment_bottom_works() {
        use nalgebra;
        use super::augment_bottom;

        let matrix_one = nalgebra::DMatrix::identity(3, 3);
        let augment_matrix = nalgebra::DMatrix::from_vec(1, 3, vec![2, 3, 4]);
        let augmented_matrix = nalgebra::Matrix4x3::new(
            1, 0, 0,
            0, 1, 0,
            0, 0, 1,
            2, 3, 4
        );
        let new_matrix = augment_bottom(matrix_one, augment_matrix);
        let new_matrix = new_matrix.expect("failed to augment");
        assert_eq!(new_matrix, augmented_matrix);
    }

    #[test]
    fn augment_right_works() {
        use nalgebra;
        use super::augment_right;

        let matrix_one = nalgebra::DMatrix::identity(3, 3);
        let augment_matrix = nalgebra::DMatrix::from_vec(3, 1, vec![2, 3, 4]);
        let augmented_matrix = nalgebra::Matrix3x4::new(
            1, 0, 0, 2,
            0, 1, 0, 3,
            0, 0, 1, 4,
        );
        let new_matrix = augment_right(matrix_one, augment_matrix);
        let new_matrix = new_matrix.expect("failed to augment");
        assert_eq!(new_matrix, augmented_matrix);
    }
}